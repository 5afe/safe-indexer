defmodule SafeIndexer.Requester do
    use GenServer

    def start_link(_) do
        # starts supervised process
        GenServer.start_link(__MODULE__, nil, name: __MODULE__)
    end

    def init(_) do
        {:ok, []}
    end

    def handle_call({:request, _url, _path}, _from, state) do
        IO.puts("Reached handler called :request");
        {:reply, :ok, state}
    end
end
