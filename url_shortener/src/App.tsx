import { useState } from 'react'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import {DeshortenerForm, ShortenerForm} from './URLForm'
import { Menu } from '@headlessui/react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function TestComponent() {
  return <></>
}


function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<TestComponent/>} path="/test_component"/>
        <Route element={
                        <Menu>
                          <Menu.Button>URL Shortener</Menu.Button>
                          <Menu.Item>
                            <ShortenerForm/>
                          </Menu.Item>
                        </Menu>
                      } path="/"/>
        <Route element={<Menu>
                          <Menu.Button> URL Deshortener </Menu.Button>
                          <Menu.Item>
                            <DeshortenerForm/>
                          </Menu.Item>
                        </Menu>} path="/detinify"/>
      </Routes>
    </BrowserRouter>
  )
}

export default App
