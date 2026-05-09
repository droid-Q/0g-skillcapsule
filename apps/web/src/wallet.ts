export interface EthereumProvider {
  isMetaMask?: boolean;
  request<T = unknown>(args: { method: string; params?: unknown[] }): Promise<T>;
  on?(event: "accountsChanged", handler: (accounts: string[]) => void): void;
  removeListener?(event: "accountsChanged", handler: (accounts: string[]) => void): void;
}

declare global {
  interface Window {
    ethereum?: EthereumProvider;
  }
}

export function getMetaMaskProvider(): EthereumProvider | null {
  return typeof window !== "undefined" && window.ethereum?.isMetaMask ? window.ethereum : null;
}

export async function connectMetaMask(): Promise<string> {
  const provider = getMetaMaskProvider();
  if (!provider) {
    throw new Error("MetaMask is not installed.");
  }
  const accounts = await provider.request<string[]>({ method: "eth_requestAccounts" });
  const account = accounts[0];
  if (!account) {
    throw new Error("No MetaMask account was returned.");
  }
  return account;
}

export async function getConnectedMetaMaskAccount(): Promise<string | null> {
  const provider = getMetaMaskProvider();
  if (!provider) return null;
  const accounts = await provider.request<string[]>({ method: "eth_accounts" });
  return accounts[0] ?? null;
}

export function formatWalletAddress(address: string): string {
  if (address.length <= 12) return address;
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

