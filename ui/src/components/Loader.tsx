'use client';

import { useEffect, useState } from "react";
import Header from "./Header";
import { useRecoilState } from "recoil";
import { refreshIdState, userState } from "@/state/auth";
import { login, logout, refresh, refreshLoggedIn, register } from "@/api/auth";
import { getFavorites, getPicture } from "@/api/users";
import Cookies from "js-cookie";
import { favoritesState, profilePictureState } from "@/state/user";
import { notifications } from "@mantine/notifications";
import { usePathname, useRouter } from "next/navigation";

export default function Loader({ children }: { children: any }) {
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useRecoilState(userState);
  const [refreshId, setRefreshId] = useRecoilState(refreshIdState);
  const [_, setFavorites] = useRecoilState(favoritesState);
  const [profilePicture, setProfilePicture] = useRecoilState(profilePictureState);
  const path = usePathname();
  const router = useRouter();

  useEffect(() => {
    setLoading(true);
    if (!user || !Cookies.get("logged_in")) {
      refreshUser();
    }
    setLoading(false);
  }, [user]);

  useEffect(() => {
    const p = path.split('/');
    console.log(p[1], user);

    if (p.length > 1) {
      if (p[1] == 'admin' && user?.role != 'admin') {
        router.push('/');
      } else if (p[1] == 'profile' && !user) {
        router.push('/');
      }
    }
  }, [path]);

  function refreshUser() {
    refresh().then((response) => {
      if (response) {
        setRefreshId(refreshLoggedIn());
        setUser(response.user);
        getFavorites().then((response) => {
          if (response) {
            setFavorites(response);
          }
        });
        if (response.user.profile_picture) {
          getPicture().then((response) => {
            if (response) {
              setProfilePicture(response as File);
            }
          });
        }
      }
    });
  }

  async function loginUser({ email, password }: { email: string, password: string}): Promise<boolean> {
    const loginResponse = await login(email, password);
    if (loginResponse) {
      setUser(loginResponse.user);
      setRefreshId(refreshLoggedIn());
      notifications.show({
        title: `Welcome back ${loginResponse.user.first_name}!`,
        message: `You have been logged in.`,
        color: 'green',
        autoClose: 2000,
        loading: false
      });
      return true;
    } else {
      notifications.show({
        title: `Unable to Login`,
        message: `Please try again.`,
        color: 'red',
        autoClose: 2000,
        loading: false
      });
    }
    return false
  }

  async function logoutUser(): Promise<void> {
    await logout();
    Cookies.remove("logged_in");
    setUser(undefined);
    setFavorites([]);
    setProfilePicture(undefined);
    if (refreshId) {
      clearInterval(refreshId);
      setRefreshId(undefined);
    }
  }

  async function registerUser({ firstName, lastName, email, password }: { firstName: string, lastName: string, email: string, password: string }): Promise<boolean> {
    const id = notifications.show({
      loading: true,
      title: `Creating account`,
      message: `Please wait...`,
      autoClose: false,
      withCloseButton: false
    });
    const registerResponse = await register({
      first_name: firstName,
      last_name: lastName,
      email: email,
      password: password
    });
    if (registerResponse) {
      const loginResponse = await login(email, password);
      if (loginResponse) {
        setUser(loginResponse.user);
        setRefreshId(refreshLoggedIn());
        notifications.update({
          id,
          title: `Account created`,
          message: `Welcome ${loginResponse.user.first_name}!`,
          color: 'green',
          autoClose: 2000,
          loading: false
        });
        return true;
      } else {
        notifications.update({
          id,
          title: `Unable to Login`,
          message: `Please try again.`,
          color: 'red',
          autoClose: 2000,
          loading: false
        });
      }
    } else {
      notifications.update({
        id,
        title: `Unable to Register`,
        message: `Please try again.`,
        color: 'error',
        autoClose: 2000,
        loading: false
      });
    }
    return false;
  }

  return (
    <>
      {loading ? (
        <></>
      ) : (
        <>
          <Header user={user} profilePicture={profilePicture} setProfilePicture={setProfilePicture} login={loginUser} logout={logoutUser} register={registerUser} />
          {children}
        </>
      )}
    </>
  )
}
