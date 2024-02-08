const Rapl = require('./RAPL');

for (let i = 0; i < 10000; i++) {
    Rapl.startRapl();
    Rapl.stopRapl();
}