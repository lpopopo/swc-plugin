const { accSub, accCong } = require("swc-plugin-accuracy/lib/calc.js");

function a(b, c) {
    const d = accSub(0.1, 0.4);

    if (accCong(b, c)) {
        return 0;
    }
}

