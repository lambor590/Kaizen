import {
  Routes,
  Route,
  NavLink,
} from "react-router-dom";
import { checkUpdate } from "@tauri-apps/api/updater";
import { useState, useEffect } from "preact/hooks";
import UpdateModal from "@/components/UpdateModal";
import Settings from "@/pages/Settings";
import Home from "@/pages/Home";
import Cleaner from "@/pages/tools/Cleaner";
import Roadmap from "@/pages/Roadmap";


export default function App() {
  const [updateAvailable, setUpdateAvailable] = useState(false);

  useEffect(() => {
    (async () => {
      const { shouldUpdate } = await checkUpdate();
      if (shouldUpdate) setUpdateAvailable(true);
    })()
  }, [])

  return (
    <div>
      {updateAvailable && <UpdateModal />}
      <div className="drawer lg:drawer-open">
        <input id="drawer" type="checkbox" className="drawer-toggle" />
        <div className="drawer-content">
          <div className="min-h-12 m-4 sticky top-4 lg:m-0">
            <label htmlFor="drawer" className="drawer-button btn lg:hidden">
              <svg className="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" strokeLinecap="round" strokeWidth="1.5" d="M5 7h14M5 12h14M5 17h14" />
              </svg>
            </label>
          </div>
          <div className="px-40 lg:px-20 pb-10 max-w-4xl">
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/tools/cleaner" element={<Cleaner />} />
              <Route path="/settings" element={<Settings />} />
              <Route path="/roadmap" element={<Roadmap />} />
            </Routes>
          </div>
        </div>
        <div className="drawer-side">
          <label htmlFor="drawer" aria-label="close sidebar" className="drawer-overlay"></label>
          <ul className="menu bg-base-200 text-base-content min-h-full w-80 p-4">
            <svg className="w-auto h-14 mb-4 drop-shadow-[0_0_20px_rgb(103,58,184)] logo-animation-hover" width="512" height="512" viewBox="0 0 512 512" fill="none" xmlns="http://www.w3.org/2000/svg">
              <g clip-path="url(#clip0_2_2)">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M325 0H269V0.000787043L55.5 318.663V141.5H-4.31979e-05L-6.10352e-05 401.5L-6.09754e-05 512L306.564 57.5H325V0ZM55 0H-4.31979e-05V90.5H55.5V58H128V57.5H130.951L169.5 0.00277539V0H128H55.5H55ZM375 0H512L473.707 57.3542L473.697 57.3697L341.08 256L512 512H444.5L363 512V453H404.859L272.5 256L405.117 57.3697H375V0ZM185.159 389.297L267.5 511.997V512H325.5V453H302.34L222.268 333.681L185.159 389.297Z" fill="url(#paint0_linear_2_2)" className="logo-animation" />
              </g>
              <defs>
                <linearGradient id="paint0_linear_2_2" x1="551.1" y1="465.966" x2="-30.8154" y2="58.2948" gradientUnits="userSpaceOnUse">
                  <stop stop-color="#434343" />
                  <stop offset="1" stop-color="white" />
                </linearGradient>
                <clipPath id="clip0_2_2">
                  <rect width="512" height="512" fill="white" />
                </clipPath>
              </defs>
            </svg>
            <li>
              <NavLink to="/">
                <svg className="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                  <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="m4 12 8-8 8 8M6 10.5V19a1 1 0 0 0 1 1h3v-3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v3h3a1 1 0 0 0 1-1v-8.5" />
                </svg>
                Inicio
              </NavLink>
            </li>
            <li>
              <details open>
                <summary>
                  <svg className="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                    <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="m10.051 8.102-3.778.322-1.994 1.994a.94.94 0 0 0 .533 1.6l2.698.316m8.39 1.617-.322 3.78-1.994 1.994a.94.94 0 0 1-1.595-.533l-.4-2.652m8.166-11.174a1.366 1.366 0 0 0-1.12-1.12c-1.616-.279-4.906-.623-6.38.853-1.671 1.672-5.211 8.015-6.31 10.023a.932.932 0 0 0 .162 1.111l.828.835.833.832a.932.932 0 0 0 1.111.163c2.008-1.102 8.35-4.642 10.021-6.312 1.475-1.478 1.133-4.77.855-6.385Zm-2.961 3.722a1.88 1.88 0 1 1-3.76 0 1.88 1.88 0 0 1 3.76 0Z" />
                  </svg>
                  Herramientas
                </summary>
                <ul>
                  <li>
                    <NavLink to="/tools/cleaner">Liberador de espacio</NavLink>
                  </li>
                </ul>
              </details>
            </li>
            <li className="disabled btn-disabled">
              <NavLink to="/settings">
                <svg className="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                  <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M21 13v-2a1 1 0 0 0-1-1h-.757l-.707-1.707.535-.536a1 1 0 0 0 0-1.414l-1.414-1.414a1 1 0 0 0-1.414 0l-.536.535L14 4.757V4a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v.757l-1.707.707-.536-.535a1 1 0 0 0-1.414 0L4.929 6.343a1 1 0 0 0 0 1.414l.536.536L4.757 10H4a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h.757l.707 1.707-.535.536a1 1 0 0 0 0 1.414l1.414 1.414a1 1 0 0 0 1.414 0l.536-.535 1.707.707V20a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-.757l1.707-.708.536.536a1 1 0 0 0 1.414 0l1.414-1.414a1 1 0 0 0 0-1.414l-.535-.536.707-1.707H20a1 1 0 0 0 1-1Z" />
                  <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
                </svg>
                Ajustes
              </NavLink>
            </li>
            <li>
              <NavLink to="/roadmap">
                <svg className="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                  <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M10 11h2v5m-2 0h4m-2.592-8.5h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                </svg>
                Próximas actualizaciones
              </NavLink>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}