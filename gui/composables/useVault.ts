import { invoke } from '@tauri-apps/api/core'

export interface Account {
  username: string
  password: string
}

export interface VaultEntry {
  service: string
  accounts: Account[]
}

export const useVault = () => {
  const listEntries = () =>
    invoke<VaultEntry[]>('list_entries')

  const addAccount = (service: string, username: string, password: string) =>
    invoke<void>('add_account', { service, username, password })

  const updateAccount = (service: string, username: string, newPassword: string) =>
    invoke<void>('update_account', { service, username, newPassword })

  const copyPassword = (service: string, username: string) =>
    invoke<void>('copy_password', { service, username })

  const generatePassword = (length: number, case_: string, useSpecials: boolean) =>
    invoke<string>('generate_password_cmd', { length, case: case_, useSpecials })

  const removeAccount = (service: string, username?: string) =>
    invoke<void>('remove_account', { service, username: username ?? null })

  return {
    listEntries,
    addAccount,
    updateAccount,
    copyPassword,
    generatePassword,
    removeAccount,
  }
}
