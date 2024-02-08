const os = require("os");
const koffi = require('koffi');

class Rapl {
    constructor() {
        const libPath = os.platform() == "win32" ? "rapl_lib.dll" : "librapl_lib.so";
        this.lib = koffi.load(libPath);
        this.start = this.lib.func('int start_rapl()');
        this.stop = this.lib.func('void stop_rapl()');
    }

    startRapl() {
        this.start();
    }

    stopRapl() {
        this.stop();
    }
}

module.exports = new Rapl();