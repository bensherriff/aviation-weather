import { useState } from 'react';
import { Avatar, Box, Burger, Button, Group, Text } from '@mantine/core';
import { useDisclosure, useToggle } from '@mantine/hooks';
import classes from './Header.module.css';
import { HeaderModal } from '@components/Header/HeaderModal.tsx';
import { notifications } from '@mantine/notifications';
import Cookies from 'js-cookie';
import { User } from '@lib/account.types.ts';
import { login, logout, register } from '@lib/account.ts';
import HeaderUser from '@components/Header/HeaderUser.tsx';

// const links = [
//   { link: '/', label: 'Map' },
//   { link: '/airports', label: 'Airports' },
//   { link: '/metars', label: 'Metars' }
// ];

export function Header() {
  const [opened, { toggle }] = useDisclosure(false);
  const [modalType, modalToggle] = useToggle([undefined, 'login', 'register', 'reset']);
  const [user, setUser] = useState<User | undefined>(undefined);
  // const [active, setActive] = useState(links[0].link);

  // const navItems = links.map((link) => (
  //   <a
  //     key={link.label}
  //     href={link.link}
  //     className={classes.link}
  //     data-active={active === link.link || undefined}
  //     onClick={(event) => {
  //       event.preventDefault();
  //       setActive(link.link);
  //     }}
  //   >
  //     {link.label}
  //   </a>
  // ));

  async function loginUser({ email, password }: { email: string; password: string }): Promise<boolean> {
    const loginResponse = await login(email, password);
    if (loginResponse) {
      setUser(loginResponse);
      notifications.show({
        title: `Welcome back ${loginResponse.first_name}!`,
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
    return false;
  }

  async function logoutUser(): Promise<void> {
    await logout();
    Cookies.remove('logged_in');
    setUser(undefined);
  }

  async function registerUser({
    firstName,
    lastName,
    email,
    password
  }: {
    firstName: string;
    lastName: string;
    email: string;
    password: string;
  }): Promise<boolean> {
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
        setUser(loginResponse);
        notifications.update({
          id,
          title: `Account created`,
          message: `Welcome ${loginResponse.first_name}!`,
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

  console.log(Cookies.get('logged_in'));
  console.log(Cookies.get('session'));

  return (
    <>
      <Box>
        <header className={classes.header}>
          <Group justify='space-between' h='100%'>
            <Group align='center' gap='xs'>
              <Burger opened={opened} onClick={toggle} hiddenFrom='xs' size='sm' />
              <Avatar src='/logo.svg' alt='logo' />
              <Text>Aviation Data</Text>
            </Group>
            {/*<Group gap={5} visibleFrom='xs' className={classes.navGroup}>*/}
            {/*  {navItems}*/}
            {/*</Group>*/}
            <Group align='center' gap='xs'>
              {user ? (
                <HeaderUser user={user} profilePicture={undefined} logout={logoutUser} />
              ) : (
                <Group className={'user'}>
                  <Button variant='default' onClick={() => modalToggle('login')}>
                    Login
                  </Button>
                  <Button onClick={() => modalToggle('register')}>Signup</Button>
                </Group>
              )}
            </Group>
          </Group>
        </header>
      </Box>
      <HeaderModal type={modalType} toggle={modalToggle} login={loginUser} register={registerUser} />
    </>
  );
}
