console.log("hello runjs", "qq");
declare const edon: any;
const path = "./log.txt";
async function main() {
  try {
    const contents = await edon.readFile(path);
    console.log("Read from a file", contents);
  } catch (err) {
    console.error("Unable to read file", path, err);
  }

  await edon.writeFile(path, "I can write to a file.");
  const contents = await edon.readFile(path);
  console.log("Read from a file", path, "contents:", contents);
  console.log("Removing file", path);
  edon.removeFile(path);
  console.log("File removed");

  const res = await edon.fetch("https://www.example.com/");
  console.log("Fetch result", res);

  interface Foo {
    bar: string;
    fizz: number;
  }
  let content: string;
  content = await edon.fetch(
    "https://deno.land/std@0.177.0/examples/welcome.ts"
  );
  console.log("Content from fetch", content);
  edon.setTimeout(() => {
    console.log("setTimeout called");
  }, 5000);
}
main();
