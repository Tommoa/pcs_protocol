# The test server <-> question server protocol

## Requirements for spec to cover
- Some form of handshake to begin. This allows us to verify our judges.
- A way to query available questions on the judge.
 - We could get around this by just having them all on the server
and just passing test cases to the judge to test.
   - Probably horribly inefficient.
- A way to update the list of available questions.
- Request to mark.
- Responses to compilation and test cases.
 - Return incorrect answer in case of fail.

## General
- All messages are little-endian.
- All integers are unsigned unless otherwise specified.
- The basic message format is as follows:
- A 2 byte message type field,
- Some number of additional message fields, the size and number of
which is dependant on the message type, and
- For some message types, a 4 byte length field and then that number
of additional data bytes.

## Message types
The message type field is a 2 byte unsigned field with the following
possible values:

`MSG-VERIFY    = 0`,
`MSG-ACCEPT    = 1`,
`MSG-DECLINE   = 2`,
`MSG-KEY       = 3`,
`MSG-REQUIRE   = 4`,
`MSG-SATISFY   = 5`,
`MSG-NOSATISFY = 6`,
`MSG-GIVE      = 7`,
`MSG-MARK      = 8`,
`MSG-MARKED    = 9`,

## Handshake
1. Question server connects via TCP as a client to the test server.
2. TLS handshake, with both sides exchanging certificates.
3. Server sends an `MSG-VERIFY`.
4. Client replies `MSG-KEY` with a string containing the key.
5. Server responds either `MSG-ACCEPT` or `MSG-DECLINE` depending on a matching
key.

`MSG-VERIFY`: `u16`
`MSG-DECLINE`: `u16`
`MSG-ACCEPT`: `u16`
`MSG-KEY`: `u16`, len: `u32`, key: `char[]`

## Question generation
1. Before sending a question to be marked, the server queries the client as to
whether the client has it by sending `MSG-REQUIRE` with the hash of the 
question.
2. Client responds `MSG-SATISFY` if it has the question or `MSG-NOSATISFY` if
it doesn't
3. If the client responds with `MSG-NOSATISFY`, the server sends `MSG-GIVE` with
the question text and test-cases.
4. Once the new question has been processed by the judge, it sends `MSG-SATISFY`

`MSG-REQUIRE`: `u16`, len: `u32`, hash: `[u8;len]`
`MSG-SATISFY`: `u16`, len: `u32`, hash: `[u8;len]`
`MSG-NOSATISFY`: `u16`, len: `u32`, hash: `[u8;len]`
`MSG-GIVE`: `u16`, len: `u32`, text: `char[]`

## Question marking
1. Server sends `MSG-MARK` to judge with a unique batch ID
2. Client marks and sends responses as they arrive with the same batch ID

`MSG-MARK`: `u16`, batch-id: `u32`, hash_len: `u32`, hash: `char[]`,
len: `u32`, answer: `char[]`
`MSG-MARKED`: `u16`, batch-id: `u32`, case: `u32`, result: `u8`,
(optional) len: `u32`, (optional) text: `char[]`, (optional) time: `[i32;2]`, 
(optional) syscall: `u64`

`MSG-MARKED`'s result field will have the following values:
0. Fail
  - Optional len and text fields activated
1. Success
  - Optional time field activated
2. Compile error (CE)
  - Optional len and text fields activated
3. Run time error (RTE)
4. Time limit exceeded (TLE)
5. Blocked syscall
  - Optional syscall field activated

## Concurrency
While the protocol guarantees that the above sequences occur in-order
for each question/batch of questions, multiple batches might be in
progress at once and so sequences might be interleaved (hence the use
of batch IDs).

Additionally, the question server makes no guarantee
that the order of messages (including `MSG-MARK`s) is the order in which
they are returned. For example, under the current question server
implementation, it marks all multiple choice questions synchronously
in-order but delegates programming questions asynchronously to another
thread. At current they are then marked one-by-one in order, but this
too might change.

Consequently:

- The server must be prepared to receive a response to _any_
of the outstanding requests it has previously sent to the client.
- The client must be prepared for the server to send many requests at
once.
