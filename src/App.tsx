import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'react'
import styles from './App.module.scss'

function App() {

  useEffect(() => {
    invoke('get_system_theme')
      .then((v) => {
        console.log("theme", v)
      })
  }, [])

  return (
    <div className={`${styles.app} theme-dark`}>
      hello
    </div>
  )
}

export default App
