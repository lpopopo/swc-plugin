const { accSub, accCong } = require("babel-plugin-accuracy/src/calc.js");

function a(b, c) {
    const d = accSub(0.1, 0.4);

    if (accCong(b, c)) {
        return 0;
    }
}

