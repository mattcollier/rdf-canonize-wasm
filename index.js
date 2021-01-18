const x = require('./pkg/rdf_canonize_wasm.js');
const quadsMergeEvent = require('./quads-merge-event.json');

const result = x.canonize(...quadsMergeEvent);
console.log('RRRRrrr', result);
