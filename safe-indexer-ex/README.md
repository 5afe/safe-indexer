# SafeIndexer

`RequestScheduler` is a dumb scheduler issuing the worker `Requester` every one second to do something. 

The current implementation is synchronous (meaning we need to handle a process response) because we use `GenServer.call`. If we want to switch over to an asynchronous implementation we should use `GenServer.cast` however, back-pressure and backing off should be considered to not overwhelm the `Requester`

https://hexdocs.pm/elixir/1.12/GenServer.html

Additionally, we have only a single `Requester` worker that handles issued requests for work from our `RequestScheduler`. We should use [Poolboy](https://elixirschool.com/en/lessons/misc/poolboy/) to make a pool of workers available to our scheduler.

## Run

```
 $ iex -S mix // interactive mode

 $ mix escript.build && ./safe_indexer // script one off run
```
