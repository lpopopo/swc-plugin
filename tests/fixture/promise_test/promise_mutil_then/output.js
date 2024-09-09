a.then(function(resolve) {
    resolve();
}).then(() => {
    return 0;
}).catch((err) => {
    console.error(err);
})