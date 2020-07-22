import React from 'react';
import clsx from 'clsx';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import useBaseUrl from '@docusaurus/useBaseUrl';
import styles from './styles.module.css';

const features = [
  {
    title: <>Yet another compile to js language</>,
    description: (
      <>
        Yes, it is yet another language that compiles to javascript but threat
        not for wasm support is planned for the not so distant future!
      </>
    ),
  },
  {
    title: <>Cool syntax 😎</>,
    description: (
      <>
        Whistle provides the coolest syntax and will not let you down unless you
        are trying to do an actual project.
      </>
    ),
  },
  {
    title: <>A language designed for the web</>,
    description: (
      <>
        The language is designed for easy interoperability with js and wasm making
        it perfect for very few things in its current state, but this will hopefully
        change soon.
      </>
    ),
  },
];

function Feature({title, description}) {
  return (
    <div className={clsx('col col--4', styles.feature)}>
      <h3>{title}</h3>
      <p>{description}</p>
    </div>
  );
}

function Home() {
  const context = useDocusaurusContext();
  const {siteConfig = {}} = context;
  return (
    <Layout
      title={`Hello from ${siteConfig.title}`}
      description="Description will go into a meta tag in <head />">
      <header className={clsx('hero hero--primary', styles.heroBanner)}>
        <div className="container">
          <h1 className="hero__title">{siteConfig.title}</h1>
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          <div className={styles.buttons}>
            <Link
              className={clsx(
                'button button--secondary button--lg',
                styles.getStarted,
              )}
              to={useBaseUrl('docs/')}>
              Get Started
            </Link>
          </div>
        </div>
      </header>
      <main>
        {features && features.length > 0 && (
          <section className={styles.features}>
            <div className="container">
              <div className="row">
                {features.map((props, idx) => (
                  <Feature key={idx} {...props} />
                ))}
              </div>
            </div>
          </section>
        )}
      </main>
    </Layout>
  );
}

export default Home;
