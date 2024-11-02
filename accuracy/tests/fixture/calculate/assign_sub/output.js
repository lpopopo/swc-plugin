const { accSub, accCong } = require("swc-plugin-accuracy/lib/calc.js");

function a(b, c) {
    let d = 0;
    d = accSub(d, 0.3);

    if (accCong(b, c)) {
        return 0;
    }
}

