declare const erode: {
  readFile: (path: string) => Promise<string>;
  writeFile: (path: string, context: string) => Promise<void>;
  removeFile: (path: string) => Promise<void>;
  fetch: (url: string) => Promise<string>;
};
