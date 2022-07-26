import { createContext, Dispatch, FC, SetStateAction, useState } from "react"


interface CurrentOperationContextValues {
  operation: string,
  setOperation: Dispatch<SetStateAction<string>>
}

export const CurrentOperationContext = createContext<CurrentOperationContextValues>({
  operation: "",
  setOperation: () => { }
})

interface CurrentOperationProviderProps {
  children: React.ReactNode
}

export const CurrentOperationProvider: FC<CurrentOperationProviderProps> = ({ children }) => {
  const [operation, setOperation] = useState<string>("")


  return (
    <CurrentOperationContext.Provider
      value={{
        operation,
        setOperation,
      }}
    >
      {children}
    </CurrentOperationContext.Provider>
  )
}
