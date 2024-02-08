const os = require("os");
const koffi = require('koffi');

class Rapl {
    constructor() {
        const libPath = os.platform() == "win32" ? "rapl_lib.dll" : "librapl_lib.so";
        this.lib = koffi.load(libPath);
        this.start = this.lib.func('int start_rapl(const char*)');
        this.stop = this.lib.func('void stop_rapl(const char*)');
    }

    startRapl(id) {
        this.start(id);
    }

    stopRapl(id) {
        this.stop(id);
    }
}

module.exports = new Rapl();