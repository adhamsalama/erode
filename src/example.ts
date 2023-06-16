console.log("Hello, world!");
declare const erode: any;
const path = "./log.txt";
async function main() {
  try {
    const contents = await erode.readFile(path);
    console.log("Read from a file", contents);
  } catch (err) {
    console.error("Unable to read file", path, err);
  }

  await erode.writeFile(path, "I can write to a file.");
  const contents = await erode.readFile(path);
  console.log("Read from a file", path, "contents:", contents);
  console.log("Removing file", path);
  erode.removeFile(path);
  console.log("File removed");

  const res = await erode.fetch("https://www.example.com/");
  console.log("Fetch result", res);

  interface Foo {
    bar: string;
    fizz: number;
  }
  let content: string;
  content = await erode.fetch(
    "https://deno.land/std@0.177.0/examples/welcome.ts"
  );
  console.log("Content from fetch", content);
  erode.setTimeout(() => {
    console.log("setTimeout called");
  }, 5000);
  console.log("should be printed before setTiemout");
}
main();
