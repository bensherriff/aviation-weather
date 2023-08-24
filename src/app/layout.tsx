import React from 'react';
import RecoilRootWrapper from '@app/recoil-root-wrapper';

import 'styles/globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
    return (
        <html lang="en">
            <body className='bg-gray-200'>
                <RecoilRootWrapper>{children}</RecoilRootWrapper>
            </body>
        </html>
    );
}
