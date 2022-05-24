import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'react'
import styles from './App.module.scss'

function App() {
  const [inputValue, setInputValue] = useState('')

  useEffect(() => {
    invoke('get_system_theme')
      .then((v) => {
        console.log(v)
      })
  }, [])

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value)
  }

  return (
    <div className={styles.app}>
      <input type="text"
        value={inputValue}
        onChange={onChange}
      />
    </div>
  )
}

export default App
