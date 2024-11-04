const { accMul, accCong } = require("swc-plugin-accuracy/lib/calc.js");

function a(b, c) {
    const d = accMul(0.1, 0.4);

    if (accCong(b, c)) {
        return 0;
    }
}

