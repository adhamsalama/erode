console.log("Hello, world!");
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
  console.log("Fetch result", res.text());

  interface Foo {
    bar: string;
    fizz: number;
  }
  let response: ErodeResponse;
  response = await erode.fetch("https://jsonplaceholder.typicode.com/posts", {
    method: "post",
  });
  console.log("Content from fetch", response.status, response.json());
  setTimeout(console.log, 5000, "Hello from setTimeout");
  console.log("should be printed before setTiemout");
}
main();
