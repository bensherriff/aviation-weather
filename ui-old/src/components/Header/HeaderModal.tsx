'use client';

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
import Cookies from 'js-cookie';

interface HeaderModalProps {
  type?: string;
  toggle: any;
  login: ({ email, password }: { email: string, password: string }) => Promise<boolean>;
  register: ({ firstName, lastName, email, password }: { firstName: string, lastName: string, email: string, password: string }) => Promise<boolean>;
}

export function HeaderModal({ type, toggle, login, register }: HeaderModalProps) {
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
      email: Cookies.get('email') || '',
      password: '',
      remember: Cookies.get('remember') === 'true'
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
                const success = await register(values);
                if (success) {
                  onClose();
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
                Cookies.set('remember', 'true', { expires: 365 });
                if (values.remember) {
                  Cookies.set('email', values.email, { expires: 365 });
                } else {
                  Cookies.remove('email');
                }
                const success = await login(values);
                if (success) {
                  onClose();
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
                <Checkbox label='Remember me' defaultChecked={loginForm.values.remember} {...loginForm.getInputProps('remember')} />
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
