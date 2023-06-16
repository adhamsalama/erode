declare const erode: {
  readFile: (path: string) => Promise<string>;
  writeFile: (path: string, context: string) => Promise<void>;
  removeFile: (path: string) => Promise<void>;
  fetch: (
    url: string,
    init?: {
      method?: string;
      headers?: Record<string, string>;
      body?: string;
    }
  ) => Promise<ErodeResponse>;
};

declare const Deno: {
  core: {
    ops: {
      op_fetch: (
        url: string,
        init?: {
          method?: string;
          headers?: Record<string, string>;
          body?: string;
        }
      ) => Promise<RFetchResponse>;
    };
  };
};

interface RFetchResponse {
  status: number;
  body: number[];
}

interface ErodeResponse {
  status: number;
  body: Uint8Array;
  json(): any;
  text(): string;
}
