# The test server <-> question server protocol

## General
- All messages are little-endian and packed for simplicity.
- All integers are unsigned unless otherwise specified.
- The basic message format is as follows:
  - A 2 byte message type field,
  - Some number of additional message fields, the size and number of
    which is dependant on the message type, and
  - For some message types, a 4 byte length field and then that number
    of additional data bytes.
- Herein, refer to "server" to mean the test server and "client" to
  mean a question server.

## Message types
The message type field is a 2 byte unsigned field with the following
possible values:

    MSG-VERIFY = 0,
    MSG-HASH = 1,
    MSG-DECLINE = 2,
    MSG-ACCEPT = 3,
    MSG-NEED = 4,
    MSG-STATUS = 5,
    MSG-GIVE = 6,
    MSG-MARK = 7,
    MSG-MARKING = 8,
    MSG-DONE = 9,

## Handshake
1. Question server connects via TCP as a client to the test server.
2. SSL handshake, with both sides exchanging certificates.
3. Server sends an MSG-VERIFY.
4. Client replies MSG-HASH with 32 bytes uniquely identifying the
   question set it is using. This allows the server to ensure all
   clients have the same question list.
5. If the hash doesn't match that of all other connected clients, the
   server sends MSG-DECLINE and expects the client to disconnect.
6. Otherwise, the server sends MSG-ACCEPT and begins using the client
   generate and mark questions.
7. Alternately, if the server doesn't support multiple question
   server clients, it might discard the hash and always send MSG-ACCEPT.

        MSG-VERIFY: u16
        MSG-HASH: u16, padding: u16, hash_value: byte[32]
        MSG-DECLINE: u16
        MSG-ACCEPT: u16

## Question generation

        MSG-NEED: u16, padding: u16, batch-id: u32, n-questions: u32
        MSG-STATUS: u16, ok: bool16, batch-id: u32,
        MSG-GIVE: u16, sequence-no: u16, batch-id: u32,
            n-answers: u32, text-length: u32, question-text: char[],
            \0, (optional, repeated) answer-text: char[], \0
        
1. If the server needs a new set of questions to display to a user, it
   should sent MSG-NEED to a client, specifying a number of questions
   and a unique batch ID that will be used to identify this set in all
   further messages.
2. The client will first respond with MSG-STATUS to inform the server
   whether it can service the request (has enough questions, etc.).
3. If the MSG-STATUS indicated success, the client will then
   immediately send one MSG-GIVE for each question requested. This
   contains:
     - A sequence number which provides the server a way
       (with the batch ID) to request operations on a specific
       question.
     - The number of answers the question has (if it's multiple
       choice, otherwise 0).
     - If a programming question: a text length field and then that
       number of bytes of a null-terminated string containing the
       question text.
     - If a multiple choice question: a text length field and then
       that number of bytes which consist of `n-answers` number of
       null-terminated strings containing the question text and then
       each possible multi-choice answer concatenated.

## Question marking

        MSG-MARK: u16, sequence-no: u16, batch-id: u32, mc-answer: u64
        MSG-MARK: u16, sequence-no: u16, batch-id: u32,
            ans-length: u64, ans-text: char[], \0
        MSG-MARKING: u16, sequence-no: u16, batch-id: u32
        MSG-DONE: u16, sequence-no: u16, batch-id: u32, m-test: u64,
			time: (i32, i32), result: u16

1. If the server has a question to mark, it sends a MSG-MARK to the
   client from which it initially received the question. That message
   contains the sequence and batch numbers as well as:
   - If a multiple choice question: the user's answer as an index,
   - If a programming question: the number of bytes of the user's
     answer text (including the null terminator), and then that text
     as a null-terminated string.
2. The client sends MSG-MARKING, identifying the question to indicate
   to the server that it has begun marking that question. If the
   question is quick to mark (i.e. is a multiple choice question), this
   may be skipped.
3. Once the question is marked, the client sends a MSG-DONE with a
   boolean indicating whether the user's answer was correct.
4. After the last question is a batch is marked, the client is free to
   deallocate memory of the batch, and all further references to the
   batch will be invalid (except for reusing its id to create a new
   batch).

## Concurrency

While the protocol guarantees that the above sequences occur in-order
for each question/batch of questions, multiple batches might be in
progress at once and so sequences might be interleaved (hence the use
of batch IDs).

Additionally, the question server makes no guarantee
that the order of messages (including MSG-MARKs) is the order in which
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
