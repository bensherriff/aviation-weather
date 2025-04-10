import { useState } from 'react';
import { Avatar, Burger, Container, Group, Text } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
// import { ReactComponent as Logo } from '../../../public/logo.svg';
import classes from './Header.module.css';

const links = [
  { link: '/', label: 'Map' },
  { link: '/airports', label: 'Airports' },
  { link: '/metars', label: 'Metars' }
];

export function Header() {
  const [opened, { toggle }] = useDisclosure(false);
  const [active, setActive] = useState(links[0].link);

  const items = links.map((link) => (
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
    <header className={classes.header}>
      <Container size='md' className={classes.inner}>
        <span style={{ display: 'flex', flexDirection: 'row' }}>
          <Text>Aviation Weather</Text>
          <Avatar src='../../../public/logo.svg' alt="it's me" />
        </span>
        {/*<Logo />*/}
        <Group gap={5} visibleFrom='xs'>
          {items}
        </Group>

        <Burger opened={opened} onClick={toggle} hiddenFrom='xs' size='sm' />
      </Container>
    </header>
  );
}
