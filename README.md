# portfolio_server
Dual purpose portfolio website, the server portion, written in Rust.

At time of writing, it is a prototype and is not running anywhere readily accessible. It involves the author learning the Rust language and relevant crates for the purposes of showcasing a musical activity the author is also actively pursing. It will serve little purpose to the community and will not include installation instructions.

## Getting from Zero to One
The author has little previous experience in Rust, though is well versed in solving the same problem in other languages (and indeed has hand spun personal creative life websites previously with services provided by other languages). The current state of the code represents going from Zero to One in Rust, and the author is well aware there are numerous software engineering concerns to be addressed before the product can be securely hosted anywhere with operational transparency.

## The Design in the Author's Head
Herein lies an opportunity for the author to choose a diagramming tool, so this text serves as a temporary stub while a visual illustration can be constructed.

The end goal is support for serving the media in properly encoded chunks from the server to a front end tool. The front end tools will be developed in a separate repository in JavaScript, and will be responsible for decoding and scheduling the chucks to provide seamless playback. Playback controls will allow for the streaming to be paused and restarted from any point in the piece's timeline. Currently the protoype downloads an entire file so it can leverage the Web Audio API's `AudioContext.decodeAudioData()` method, which only succeeds with a "complete" audio file. It provides no controls or interaction with the audio playback. Chunking, streaming, and reassembling chunks of audio is a more complex problem that requires a deeper dive into the manuals and crates to support the file stream buffer, encoding, decoding and scheduling of chunks.

This will ultimately be deployed on a server that runs on Akamai's network. The author is considering using Pulumi in Rust for provisioning infrastructure. The code for this would also be in a separate repository.

### Use of LLMs
The author has experience using "stock" LLMs and the "next level up" code agents which attempt to provide a "code savvy" layer to stock LLMs. The author has found them generally disappointing, but especially disappointing for suggesting Rust which has structure or even compiles. The choice of Rust keeps the author honest (and is of personal interest to the author for other reasons).

LLMs have been useful in providing general direction: crates to investigate, JavaScript and DOM constructs that haven't been employed in a while, and refreshing on standard Linux administration skills that haven't been exercised as often in recent professional roles.

The author has found copy/pasting Rust code suggestions from LLMs rather fruitless compared to the documentation and examples available with crates.
