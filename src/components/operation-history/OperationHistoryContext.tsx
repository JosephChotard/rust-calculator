import { tauri } from "@tauri-apps/api"
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
  addToHistory: (operation) => { },
})

interface OperationHistoryProviderProps {
  children: React.ReactNode
}

export const OperationHistoryProvider: FC<OperationHistoryProviderProps> = ({ children }) => {
  const [history, setHistory] = useState<Operation[]>([])

  const addToHistory = (operation: Operation) => {
    setHistory([...history, operation])
  }

  useEffect(() => {
    tauri.invoke<Operation[]>('get_operation_history_command').then((history) => {
      console.log(history)
      setHistory(history)
    })
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
