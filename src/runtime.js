((globalThis) => {
  const core = Deno.core;

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg));
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      core.print(`${argsToMessage(...args)}\n`, true);
    },
  };
  globalThis.setTimeout = (callback, delay, ...arguments) => {
    return core.ops.op_set_timeout(delay).then(() => callback(...arguments));
  };
  class ErodeResponse {
    constructor(responseStruct) {
      this.status = responseStruct.status;
      this.body = new Uint8Array(responseStruct.body);
    }
    json() {
      return JSON.parse(this.text());
    }
    text() {
      let strArr = [];
      for (const byte of this.body) {
        strArr.push(String.fromCharCode(byte));
      }
      return strArr.join("");
    }
  }

  globalThis.erode = {
    readFile: (path) => {
      return core.ops.op_read_file(path);
    },
    writeFile: (path, contents) => {
      return core.ops.op_write_file(path, contents);
    },
    removeFile: (path) => {
      return core.ops.op_remove_file(path);
    },
    async fetch(url, init = {}) {
      if (typeof url !== "string") {
        url = url.toString();
      }
      init = {
        method: "get",
        headers: {},
        body: null,
        ...init,
      };
      const responseStruct = await core.ops.op_fetch(
        url,
        init.method,
        init.headers,
        init.body
      );
      return new ErodeResponse(responseStruct);
    },
  };
})(globalThis);
