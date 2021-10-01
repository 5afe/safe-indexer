defmodule SafeIndexer do
  use Application

  def start(_type, _args) do
    # Defines supervisor tree
    children = [
      {SafeIndexer.Requester, []},
      {SafeIndexer.RequestScheduler, []}
    ]

    opts = [strategy: :one_for_one, name: SafeIndexer.Requester.Supervisor]

    # Starts supervisor tree
    Supervisor.start_link(children, opts)
  end
end
