defmodule SafeIndexer.RequestScheduler do
    use GenServer

    def start_link(_) do
        # starts supervised process
        GenServer.start_link(__MODULE__, nil, name: __MODULE__)
    end

    def init(_) do
        Process.send_after(self(), :check, 1000)
        {:ok, []}
    end

    def handle_info(:check, state) do
        GenServer.call(SafeIndexer.Requester, {:request, "asdf", "Asdf"}, :infinity)
        Process.send_after(self(), :check, 1000)
        {:noreply, state}
    end
end
