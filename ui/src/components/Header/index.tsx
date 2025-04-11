import { useState } from 'react';
import { Avatar, Box, Burger, Button, Group, Text } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import classes from './Header.module.css';

const links = [
  { link: '/', label: 'Map' },
  { link: '/airports', label: 'Airports' },
  { link: '/metars', label: 'Metars' }
];

export function Header() {
  const [opened, { toggle }] = useDisclosure(false);
  const [active, setActive] = useState(links[0].link);
  const isSignedIn = false;

  const navItems = links.map((link) => (
    <a
      key={link.label}
      href={link.link}
      className={classes.link}
      data-active={active === link.link || undefined}
      onClick={(event) => {
        event.preventDefault();
        setActive(link.link);
      }}
    >
      {link.label}
    </a>
  ));

  return (
    <Box>
      <header className={classes.header}>
        <Group justify='space-between' h='100%'>
          <Group align='center' gap='xs'>
            <Burger opened={opened} onClick={toggle} hiddenFrom='xs' size='sm' />
            <Avatar src='/logo.svg' alt='logo' />
            <Text>Aviation</Text>
          </Group>
          <Group gap={5} visibleFrom='xs' className={classes.navGroup}>
            {navItems}
          </Group>
          <Group align='center' gap='xs'>
            {isSignedIn ? (
              // Clickable avatar if signed in
              <Avatar
                src='/user-avatar.jpg' // replace with dynamic source when available
                alt='User avatar'
                style={{ cursor: 'pointer' }}
                // Add click handler for user dropdown if needed
              />
            ) : (
              <>
                <Button variant='default'>Login</Button>
                <Button>Signup</Button>
              </>
            )}
          </Group>
        </Group>
      </header>
    </Box>
  );
}
