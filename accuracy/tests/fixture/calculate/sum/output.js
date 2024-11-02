const { accAdd, accCong } = require("swc-plugin-accuracy/lib/calc.js");

function a(b, c) {
    const d = accAdd(0.1, 0.4);

    if (accCong(b, c)) {
        return 0;
    }
}

