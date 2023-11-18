'use client';

import { login, register, refreshLoggedIn } from '@/api/auth';
import { User } from '@/api/auth.types';
import {
  Modal,
  Container,
  Title,
  Anchor,
  Paper,
  TextInput,
  Button,
  PasswordInput,
  Group,
  Checkbox,
  Text
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { notifications } from '@mantine/notifications';

interface HeaderModalProps {
  type?: string;
  toggle: any;
  setUser: (user: User) => void;
  setRefreshId: (id: NodeJS.Timeout) => void;
}

export function HeaderModal({ type, toggle, setUser, setRefreshId }: HeaderModalProps) {
  function passwordValidator(value: string) {
    if (value.trim().length < 10) {
      return 'Password must be at least 10 characters';
    }
    if (value.trim().length >= 128) {
      return 'Password must be at most 128 characters';
    }
    if (!/(\d)/.test(value)) {
      return 'Password must contain at least one number';
    }
    if (!/[a-z]/.test(value)) {
      return 'Password must contain at least one lowercase letter';
    }
    if (!/[A-Z]/.test(value)) {
      return 'Password must contain at least one uppercase letter';
    }
    if (!/[!@#$%^&*]/.test(value)) {
      return 'Password must contain at least one special character';
    }
    return null;
  }

  function emailValidator(value: string) {
    if (value.trim().length == 0) {
      return 'Email is required';
    }
    if (!/^\S+@\S+$/.test(value)) {
      return 'Invalid email';
    }
    return null;
  }

  const registerForm = useForm({
    initialValues: {
      firstName: '',
      lastName: '',
      email: '',
      password: ''
    },
    validate: {
      firstName: (value) => (value.trim().length > 0 ? null : 'First name is required'),
      lastName: (value) => (value.trim().length > 0 ? null : 'Last name is required'),
      email: emailValidator,
      password: passwordValidator
    }
  });

  const loginForm = useForm({
    initialValues: {
      email: '',
      password: '',
      remember: false
    }
  });

  const resetForm = useForm({
    initialValues: {
      email: ''
    }
  });

  function onClose() {
    toggle(undefined);
    registerForm.reset();
    resetForm.reset();
    if (!loginForm.values.remember) {
      loginForm.reset();
    }
  }

  return (
    <Modal opened={type !== undefined} onClose={onClose} withCloseButton={false}>
      {type == 'reset' ? (
        <Container>
          <Title ta='center'>Reset password</Title>
          <Text c='dimmed' size='sm' ta='center' mt={5}>
            Enter your email and we will send you a link to reset your password.{' '}
            <Anchor size='sm' component='a' onClick={() => toggle('login')}>
              Go Back
            </Anchor>
          </Text>
          <Paper withBorder shadow='md' p={30} mt={30} radius='md'>
            <form onSubmit={resetForm.onSubmit(async (values) => console.log(values))}>
              <TextInput label='Email' placeholder='you@example.com' required {...resetForm.getInputProps('email')} />
              <Button type='submit' fullWidth mt='xl'>
                Reset password
              </Button>
            </form>
          </Paper>
        </Container>
      ) : type == 'register' ? (
        <Container>
          <Title ta='center'>Create account</Title>
          <Text c='dimmed' size='sm' ta='center' mt={5}>
            Already have an account?{' '}
            <Anchor size='sm' component='a' onClick={() => toggle('login')}>
              Sign in
            </Anchor>
          </Text>

          <Paper withBorder shadow='md' p={30} mt={30} radius='md'>
            <form
              onSubmit={registerForm.onSubmit(async (values) => {
                const id = notifications.show({
                  loading: true,
                  title: `Creating account`,
                  message: `Please wait...`,
                  autoClose: false,
                  withCloseButton: false
                });
                const registerResponse = await register({
                  first_name: values.firstName,
                  last_name: values.lastName,
                  email: values.email,
                  password: values.password
                });
                if (registerResponse) {
                  const loginResponse = await login(values.email, values.password);
                  if (loginResponse) {
                    setUser(loginResponse.user);
                    setRefreshId(refreshLoggedIn());
                    onClose();
                    notifications.update({
                      id,
                      title: `Account created`,
                      message: `Welcome ${loginResponse.user.first_name}!`,
                      color: 'green',
                      autoClose: 2000,
                      loading: false
                    });
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
              })}
            >
              <TextInput label='First name' placeholder='John' required {...registerForm.getInputProps('firstName')} />
              <TextInput
                label='Last name'
                placeholder='Smith'
                required
                mt='md'
                {...registerForm.getInputProps('lastName')}
              />
              <TextInput
                label='Email'
                placeholder='you@example.com'
                required
                {...registerForm.getInputProps('email')}
              />
              <PasswordInput
                label='Password'
                description='Passwords must be at least 10 characters long, contain at least one number, one uppercase letter, one lowercase letter, and one special character.'
                placeholder='Your password'
                required
                mt='md'
                {...registerForm.getInputProps('password')}
              />
              <Button type='submit' fullWidth mt='xl'>
                Sign up
              </Button>
            </form>
          </Paper>
        </Container>
      ) : (
        <Container>
          <Title ta='center'>Welcome back!</Title>
          <Text c='dimmed' size='sm' ta='center' mt={5}>
            Do not have an account yet?{' '}
            <Anchor size='sm' component='a' onClick={() => toggle('register')}>
              Create account
            </Anchor>
          </Text>

          <Paper withBorder shadow='md' p={30} mt={30} radius='md'>
            <form
              onSubmit={loginForm.onSubmit(async (values) => {
                const response = await login(values.email, values.password);
                if (response) {
                  setUser(response.user);
                  setRefreshId(refreshLoggedIn());
                  onClose();
                } else {
                  notifications.show({
                    title: `Unable to Login`,
                    message: `Please try again.`,
                    color: 'red',
                    autoClose: 2000
                  });
                }
              })}
            >
              <TextInput label='Email' placeholder='you@example.com' required {...loginForm.getInputProps('email')} />
              <PasswordInput
                label='Password'
                placeholder='Your password'
                required
                mt='md'
                {...loginForm.getInputProps('password')}
              />
              <Group justify='space-between' mt='lg'>
                <Checkbox label='Remember me' {...loginForm.getInputProps('remember')} />
                <Anchor component='a' size='sm' onClick={() => toggle('reset')}>
                  Forgot password?
                </Anchor>
              </Group>
              <Button type='submit' fullWidth mt='xl'>
                Sign in
              </Button>
            </form>
          </Paper>
        </Container>
      )}
    </Modal>
  );
}
