import { useEffect } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

import { PATHS } from '@src/lib/paths'
import { useAuthState } from '@src/lib/singletons'

/**
 * A simple hook that listens to the auth state of the app and navigates
 * accordingly.
 */
export function useAuthNavigation() {
  const navigate = useNavigate()
  const location = useLocation()
  const authState = useAuthState()

  // Subscribe to the auth state of the app and navigate accordingly.
  useEffect(() => {
    if (
      authState.matches('loggedIn') &&
      location.pathname.includes(PATHS.SIGN_IN)
    ) {
      navigate(PATHS.INDEX)
    } else if (
      authState.matches('loggedOut') &&
      !location.pathname.includes(PATHS.SIGN_IN)
    ) {
      navigate(PATHS.SIGN_IN + (location.search || ''))
    }
  }, [authState, location.pathname])
}
