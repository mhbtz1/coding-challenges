import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import './App.css'


function TestComponent() {
  return <></>
}

function App() {
  <BrowserRouter>
    <Routes>
      <Route element={<TestComponent/>} path="/"/>
    </Routes>
  </BrowserRouter>
}