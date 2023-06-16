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

    async fetch(url) {
      return await core.ops.op_fetch(url);
    },
  };
})(globalThis);
