* [ ] Figure out a better way to auto-download and install the library files on Windows, 'cause manual install is balls.
* [ ] Make a chat module to manage chat messages as a list of structs containing meta information about each chat (timestamps, sender ID, etc)
* [ ] Make audio module:
  * [ ] Record microphone input
  * [ ] Receive voice messages from network
  * [ ] Send voice events to network
  * [ ] Maintain a player for each Peer, with information for controlling volume (distance?)
  * [ ] Play back recorded audio
* [ ] Is there a smart way to separate out chat message streams from voice message streams on the client side?
* [ ] Turn UI views into traits and implement the trait for each controller (modules should have no knowledge about UI state ever)
* [ ] Move main menu creation to its own module