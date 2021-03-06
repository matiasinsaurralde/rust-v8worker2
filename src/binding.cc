/*
Copyright 2018 Ryan Dahl <ry@tinyclouds.org>. All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to
deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE.
*/
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string>

#include "binding.h"
#include "libplatform/libplatform.h"
#include "v8.h"

using namespace v8;

struct worker_s {
  void* rust_callback;
  void* rust_object;
  Isolate* isolate;
  std::string last_exception;
  Persistent<Function> recv;
  Persistent<Context> context;
};

// Extracts a C string from a V8 Utf8Value.
const char* ToCString(const String::Utf8Value& value) {
  return *value ? *value : "<string conversion failed>";
}

/*
bool AbortOnUncaughtExceptionCallback(Isolate* isolate) {
  return true;
}

void MessageCallback2(Local<Message> message, Local<Value> data) {
  printf("MessageCallback2\n\n");
}

void FatalErrorCallback2(const char* location, const char* message) {
  printf("FatalErrorCallback2\n");
}
*/

void ExitOnPromiseRejectCallback(PromiseRejectMessage promise_reject_message) {
  auto isolate = Isolate::GetCurrent();
  worker* w = (worker*)isolate->GetData(0);
  assert(w->isolate == isolate);
  HandleScope handle_scope(w->isolate);
  auto context = w->context.Get(w->isolate);

  auto exception = promise_reject_message.GetValue();

  auto message = Exception::CreateMessage(isolate, exception);
  auto onerrorStr = String::NewFromUtf8(w->isolate, "onerror");
  auto onerror = context->Global()->Get(onerrorStr);

  if (onerror->IsFunction()) {
    Local<Function> func = Local<Function>::Cast(onerror);
    Local<Value> args[5];
    auto origin = message->GetScriptOrigin();
    args[0] = exception->ToString();
    args[1] = message->GetScriptResourceName();
    args[2] = origin.ResourceLineOffset();
    args[3] = origin.ResourceColumnOffset();
    args[4] = exception;
    func->Call(context->Global(), 5, args);
    /* message, source, lineno, colno, error */
  } else {
    printf("Unhandled Promise\n");
    message->PrintCurrentStackTrace(isolate, stdout);
  }

  exit(1);
}

// Exception details will be appended to the first argument.
std::string ExceptionString(worker* w, TryCatch* try_catch) {
  std::string out;
  size_t scratchSize = 20;
  char scratch[scratchSize];  // just some scratch space for sprintf

  HandleScope handle_scope(w->isolate);
  Local<Context> context = w->context.Get(w->isolate);
  String::Utf8Value exception(try_catch->Exception());
  const char* exception_string = ToCString(exception);

  Handle<Message> message = try_catch->Message();

  if (message.IsEmpty()) {
    // V8 didn't provide any extra information about this error; just
    // print the exception.
    out.append(exception_string);
    out.append("\n");
  } else {
    // Print (filename):(line number)
    String::Utf8Value filename(message->GetScriptOrigin().ResourceName());
    const char* filename_string = ToCString(filename);
    int linenum = message->GetLineNumber();

    snprintf(scratch, scratchSize, "%i", linenum);
    out.append(filename_string);
    out.append(":");
    out.append(scratch);
    out.append("\n");

    // Print line of source code.
    String::Utf8Value sourceline(message->GetSourceLine());
    const char* sourceline_string = ToCString(sourceline);

    out.append(sourceline_string);
    out.append("\n");

    // Print wavy underline (GetUnderline is deprecated).
    int start = message->GetStartColumn(context).FromJust();
    for (int i = 0; i < start; i++) {
      out.append(" ");
    }
    int end = message->GetEndColumn(context).FromJust();
    for (int i = start; i < end; i++) {
      out.append("^");
    }
    out.append("\n");
    String::Utf8Value stack_trace(try_catch->StackTrace());
    if (stack_trace.length() > 0) {
      const char* stack_trace_string = ToCString(stack_trace);
      out.append(stack_trace_string);
      out.append("\n");
    } else {
      out.append(exception_string);
      out.append("\n");
    }
  }
  return out;
}

extern "C" {
// #include "_cgo_export.h"

const char* worker_version() { return V8::GetVersion(); }

void worker_set_flags(int* argc, char** argv) {
  V8::SetFlagsFromCommandLine(argc, argv, true);
}

const char* worker_last_exception(worker* w) {
  return w->last_exception.c_str();
}

int worker_load(worker* w, char* name_s, char* source_s) {
  Locker locker(w->isolate);
  Isolate::Scope isolate_scope(w->isolate);
  HandleScope handle_scope(w->isolate);

  Local<Context> context = Local<Context>::New(w->isolate, w->context);
  Context::Scope context_scope(context);

  TryCatch try_catch(w->isolate);

  Local<String> name = String::NewFromUtf8(w->isolate, name_s);
  Local<String> source = String::NewFromUtf8(w->isolate, source_s);

  ScriptOrigin origin(name);

  Local<Script> script = Script::Compile(source, &origin);

  if (script.IsEmpty()) {
    assert(try_catch.HasCaught());
    w->last_exception = ExceptionString(w, &try_catch);
    return 1;
  }

  Handle<Value> result = script->Run();

  if (result.IsEmpty()) {
    assert(try_catch.HasCaught());
    w->last_exception = ExceptionString(w, &try_catch);
    return 2;
  }

  return 0;
}

void Print(const FunctionCallbackInfo<Value>& args) {
  bool first = true;
  for (int i = 0; i < args.Length(); i++) {
    HandleScope handle_scope(args.GetIsolate());
    if (first) {
      first = false;
    } else {
      printf(" ");
    }
    String::Utf8Value str(args[i]);
    const char* cstr = ToCString(str);
    printf("%s", cstr);
  }
  printf("\n");
  fflush(stdout);
}

// Sets the recv callback.
void Recv(const FunctionCallbackInfo<Value>& args) {
  Isolate* isolate = args.GetIsolate();
  worker* w = (worker*)isolate->GetData(0);
  assert(w->isolate == isolate);

  HandleScope handle_scope(isolate);

  auto context = w->context.Get(w->isolate);

  Local<Value> v = args[0];
  assert(v->IsFunction());
  Local<Function> func = Local<Function>::Cast(v);

  w->recv.Reset(isolate, func);
}

// Called from JavaScript, routes message to golang.
void Send(const FunctionCallbackInfo<Value>& args) {
  Isolate* isolate = args.GetIsolate();
  worker* w = static_cast<worker*>(isolate->GetData(0));
  assert(w->isolate == isolate);

  Locker locker(w->isolate);
  EscapableHandleScope handle_scope(isolate);

  auto context = w->context.Get(w->isolate);

  Local<Value> v = args[0];
  assert(v->IsArrayBuffer());

  auto ab = Local<ArrayBuffer>::Cast(v);
  auto contents = ab->GetContents();

  void* buf = contents.Data();
  int buflen = static_cast<int>(contents.ByteLength());

  buf_s* retbuf = recv(buf, buflen, w->rust_callback);
  if (retbuf->data != NULL) {
    auto ab = ArrayBuffer::New(w->isolate, retbuf->len);
    auto contents = ab->GetContents();
    memcpy(contents.Data(), retbuf->data, retbuf->len);
    // TODO: investigate what's going on with the Rust FFI pointer
    // free(retbuf.data);
    args.GetReturnValue().Set(handle_scope.Escape(ab));
  }
}

// Called from golang. Must route message to javascript lang.
// non-zero return value indicates error. check worker_last_exception().
int worker_send_bytes(worker* w, void* data, size_t len) {
  Locker locker(w->isolate);
  Isolate::Scope isolate_scope(w->isolate);
  HandleScope handle_scope(w->isolate);

  auto context = w->context.Get(w->isolate);

  TryCatch try_catch(w->isolate);

  Local<Function> recv = Local<Function>::New(w->isolate, w->recv);
  if (recv.IsEmpty()) {
    w->last_exception = "V8Worker2.recv has not been called.";
    return 1;
  }

  Local<Value> args[1];
  args[0] = ArrayBuffer::New(w->isolate, data, len,
                             ArrayBufferCreationMode::kInternalized);
  assert(!args[0].IsEmpty());
  assert(!try_catch.HasCaught());
  
  recv->Call(context->Global(), 1, args);

  if (try_catch.HasCaught()) {
    w->last_exception = ExceptionString(w, &try_catch);
    return 2;
  }

  return 0;
}

void v8_init() {
  Platform* platform = platform::CreateDefaultPlatform();
  V8::InitializePlatform(platform);
  V8::Initialize();
}

worker* worker_new() {
  worker* w = new (worker);

  Isolate::CreateParams create_params;
  create_params.array_buffer_allocator =
      ArrayBuffer::Allocator::NewDefaultAllocator();
  Isolate* isolate = Isolate::New(create_params);
  Locker locker(isolate);
  Isolate::Scope isolate_scope(isolate);
  HandleScope handle_scope(isolate);

  w->isolate = isolate;
  // Leaving this code here because it will probably be useful later on, but
  // disabling it now as I haven't got tests for the desired behavior.
  // w->isolate->SetCaptureStackTraceForUncaughtExceptions(true);
  // w->isolate->SetAbortOnUncaughtExceptionCallback(AbortOnUncaughtExceptionCallback);
  // w->isolate->AddMessageListener(MessageCallback2);
  // w->isolate->SetFatalErrorHandler(FatalErrorCallback2);
  w->isolate->SetPromiseRejectCallback(ExitOnPromiseRejectCallback);
  w->isolate->SetData(0, w);

  Local<ObjectTemplate> global = ObjectTemplate::New(w->isolate);
  Local<ObjectTemplate> v8worker2 = ObjectTemplate::New(w->isolate);

  global->Set(String::NewFromUtf8(w->isolate, "V8Worker2"), v8worker2);

  v8worker2->Set(String::NewFromUtf8(w->isolate, "print"),
                 FunctionTemplate::New(w->isolate, Print));

  v8worker2->Set(String::NewFromUtf8(w->isolate, "recv"),
                 FunctionTemplate::New(w->isolate, Recv));

  v8worker2->Set(String::NewFromUtf8(w->isolate, "send"),
                 FunctionTemplate::New(w->isolate, Send));

  Local<Context> context = Context::New(w->isolate, NULL, global);
  w->context.Reset(w->isolate, context);
  context->Enter();

  return w;
}

void worker_dispose(worker* w) {
  w->isolate->Dispose();
  delete (w);
}

void worker_terminate_execution(worker* w) { w->isolate->TerminateExecution(); }
void worker_set_rust_callback(worker* w, void* rust_callback) {
  w->rust_callback = rust_callback;
}
void worker_set_rust_object(worker* w, void* rust_object) {
  w->rust_object = rust_object;
}
}