async function printFile(filename) {
    let contents = await fs.readFileAsync(filename, 'utf8');
    console.log(contents);
}
async (filename) => {
    let contents = await fs.readFileAsync(filename, 'utf8');
    console.log(contents);
}
