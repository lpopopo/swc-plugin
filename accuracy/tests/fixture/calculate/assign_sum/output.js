const { accAdd, accCong } = require("babel-plugin-accuracy/src/calc.js");

function a(b, c) {
    let d = 0;
    d = accAdd(d, 0.3);

    if (accCong(b, c)) {
        return 0;
    }
}
