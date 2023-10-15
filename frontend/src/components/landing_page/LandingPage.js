import React from "react";
import Sidebar from "./Sidebar";

export default function LandingPage() {
  return (
  <>
    <div className="w-screen h-screen bg-primary">
      <h1 className="flex">
        <Sidebar />
      </h1>
      <div className="grid h-screen place-items-center
                  font-family:Georgia text-6xl font-bold text-white">
        Masq
      </div>
    </div>
  </>
  )
}