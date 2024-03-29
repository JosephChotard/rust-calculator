import { tauri } from "@tauri-apps/api"
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { createContext, FC, useEffect, useState } from "react"

export interface Operation {
  operation: string,
  result: number
}

interface OperationHistoryContextValues {
  history: Operation[],
  addToHistory: (operation: Operation) => void
}

export const OperationHistoryContext = createContext<OperationHistoryContextValues>({
  history: [],
  addToHistory: () => { },
})

interface OperationHistoryProviderProps {
  children: React.ReactNode
}

export const OperationHistoryProvider: FC<OperationHistoryProviderProps> = ({ children }) => {
  const [history, setHistory] = useState<Operation[]>([])

  const addToHistory = (operation: Operation) => {
    setHistory(history => [...history, operation])
  }

  useEffect(() => {
    tauri.invoke<Operation[]>('get_operation_history_command').then((history) => {
      setHistory(history)
    })
    let unlisten_clearHistory: UnlistenFn
    let unlisten_addToHistory: UnlistenFn
    let isCancelled = false;
    (async () => {
      unlisten_clearHistory = await listen('history_cleared', (history) => {
        setHistory([])
      })
      unlisten_addToHistory = await listen<Operation>('add_to_history', (event) => {
        addToHistory(event.payload)
      })
      if (isCancelled) {
        unlisten_clearHistory?.()
        unlisten_addToHistory?.()
      }
    })()

    return () => {
      isCancelled = true
      unlisten_clearHistory?.()
      unlisten_addToHistory?.()
    }
  }, [])

  return (
    <OperationHistoryContext.Provider
      value={{
        history,
        addToHistory,
      }}
    >
      {children}
    </OperationHistoryContext.Provider>
  )
}
