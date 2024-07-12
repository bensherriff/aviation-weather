import { User } from "@/api/auth.types";
import { setPicture } from "@/api/users";
import {
  Menu,
  UnstyledButton,
  Group,
  Avatar,
  Card,
  FileButton,
  Grid,
  Button,
  Text
} from "@mantine/core";
import Link from "next/link";
import { SetterOrUpdater } from "recoil";
import './styles.css';

interface UserMenuProps {
  user: User;
  profilePicture: File | undefined;
  setProfilePicture: SetterOrUpdater<File | undefined>;
  toggle: (type: string) => void;
  logout: () => Promise<void>;
}

export default function UserMenu({ user, profilePicture, setProfilePicture, logout, toggle }: UserMenuProps) {

  return (
    <Menu shadow='md' width={200} openDelay={100} closeDelay={400}>
      <Menu.Target>
        <UnstyledButton>
          <Group>
            <Avatar src={profilePicture ? URL.createObjectURL(profilePicture) : undefined} />
            <div style={{ flex: 1 }}>
              <Text size='sm' fw={500}>
                {user.first_name} {user.last_name}
              </Text>
              <Text c='dimmed' size='xs' style={{ textTransform: 'uppercase' }}>
                {user.role}
              </Text>
            </div>
          </Group>
        </UnstyledButton>
      </Menu.Target>
      <Menu.Dropdown p={0}>
        <Card>
          <Card.Section h={140} style={{ backgroundColor: '#4481e3' }} />
          <FileButton
            onChange={(payload) => {
              if (payload) {
                setPicture(payload).then((response) => {
                  if (response) {
                    setProfilePicture(payload);
                  }
                });
              }
            }}
            accept='image/png,image/jpeg,image/svg+xml,image/webp,image/gif,image/apng,image/avif'
            multiple={false}
          >
            {(props) => (
              <Avatar
                {...props}
                component='button'
                size={80}
                radius={80}
                mx={'auto'}
                mt={-30}
                style={{ cursor: 'pointer' }}
                bg={profilePicture ? 'transparent' : 'white'}
                src={profilePicture ? URL.createObjectURL(profilePicture) : undefined}
              />
            )}
          </FileButton>
          <Text ta='center' fz='lg' fw={500} mt='sm'>
            {user.first_name} {user.last_name}
          </Text>
          <Text ta='center' fz='sm' c='dimmed' style={{ textTransform: 'uppercase' }}>
            {user.role}
          </Text>
          <Grid mt='xl'>
            <Grid.Col span={6}>
              <Link href='/profile'>
                <Button fullWidth radius='md' size='xs' variant='default'>
                  Profile
                </Button>
              </Link>
            </Grid.Col>
            <Grid.Col span={6}>
              <Button
                fullWidth
                radius='md'
                size='xs'
                variant='default'
                onClick={logout}
              >
                Logout
              </Button>
            </Grid.Col>
            {user.role == 'admin' && (
              <Grid.Col span={12}>
                <Link href='/admin'>
                  <Button fullWidth radius='md' size='xs' variant='default'>
                    Administration
                  </Button>
                </Link>
              </Grid.Col>
            )}
          </Grid>
        </Card>
      </Menu.Dropdown>
    </Menu>
  )
}