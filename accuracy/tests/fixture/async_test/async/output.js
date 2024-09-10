async function printFile(filename) {
    try {
        let contents = await fs.readFileAsync(filename, 'utf8');
        console.log(contents);
    } catch (error) {
        console.error(this, error);
    }
}

async filename => {
    try {
        let contents = await fs.readFileAsync(filename, 'utf8');
        console.log(contents);
    } catch (error) {
        console.error(this, error);
    }
};
