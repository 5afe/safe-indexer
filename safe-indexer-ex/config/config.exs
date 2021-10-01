use Mix.Config

config :tesla, :adapter, Tesla.Adapter.Hackney
config :safe_indexer, rpc_node_url: System.get_env("RPC_NODE_URL")
