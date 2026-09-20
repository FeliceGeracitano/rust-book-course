import { useId, useState } from 'react'
import type { QuizData, TraceData } from '../content'
import { recordAnswer, useProgress } from '../progress'
import CodeBlock from './CodeBlock'

export function Quiz({ data, lessonId }: { data: QuizData; lessonId: string }) {
  const key = `${lessonId}#${data.id}`
  const saved = useProgress().answers[key]
  const [retry, setRetry] = useState(false)
  const picked = retry ? undefined : saved?.choice
  const answered = picked !== undefined
  const correct = picked === data.answer
  const headingId = useId()
  return (
    <section className="discovery" aria-labelledby={headingId}>
      <p className="eyebrow">Try it · make a prediction</p>
      <h2 id={headingId}>{data.question}</h2>
      {data.code && (
        <CodeBlock className="language-rust">{data.code}</CodeBlock>
      )}
      <div className="grid gap-2">
        {data.options.map((option, index) => (
          <button
            key={option}
            disabled={answered}
            onClick={() => {
              recordAnswer(key, index)
              setRetry(false)
            }}
            className={`answer ${answered && index === data.answer ? 'correct' : answered && index === picked ? 'incorrect' : ''}`}
          >
            <span className="mr-3 text-muted">
              {String.fromCharCode(65 + index)}.
            </span>{' '}
            {option}
            {answered && index === data.answer && <span> ✓</span>}
          </button>
        ))}
      </div>
      {answered && (
        <div className="mt-4" role="status">
          <p className={correct ? 'text-ok' : 'text-crab'}>
            {correct
              ? 'Correct — here’s why.'
              : 'Not quite — follow the reasoning.'}
          </p>
          <p className="mt-2 leading-relaxed">{data.explain}</p>
          <button className="text-link mt-3" onClick={() => setRetry(true)}>
            Try again
          </button>
          <span className="ml-4 text-xs text-muted">
            {saved?.attempts} attempt{saved?.attempts === 1 ? '' : 's'}
          </span>
        </div>
      )}
    </section>
  )
}

export function Trace({ data }: { data: TraceData }) {
  const [index, setIndex] = useState(0)
  const step = data.steps[index]
  const headingId = useId()
  return (
    <section className="discovery" aria-labelledby={headingId}>
      <p className="eyebrow">Explore · step through the example</p>
      <h2 id={headingId}>{data.title}</h2>
      <p className="mb-3 text-sm text-muted">
        An illustrated walkthrough of this example.
      </p>
      <div className="overflow-x-auto rounded-lg border border-edge bg-ink p-3">
        <pre>
          {data.code.split('\n').map((line, i) => (
            <div
              key={i}
              className={i + 1 === step.line ? 'bg-rust/20 text-crab' : ''}
              aria-current={i + 1 === step.line ? 'step' : undefined}
            >
              <span className="mr-4 inline-block w-5 text-right text-muted">
                {i + 1}
              </span>
              {line || ' '}
            </div>
          ))}
        </pre>
      </div>
      <div className="my-4 flex flex-wrap items-center gap-3">
        <button
          className="secondary"
          disabled={index === 0}
          onClick={() => setIndex(index - 1)}
        >
          Previous step
        </button>
        <span className="text-sm text-muted">
          {index + 1} / {data.steps.length}
        </span>
        <button
          className="secondary"
          disabled={index === data.steps.length - 1}
          onClick={() => setIndex(index + 1)}
        >
          Next step
        </button>
        <button className="text-link" onClick={() => setIndex(0)}>
          Restart
        </button>
      </div>
      <div aria-live="polite">
        <p>{step.note}</p>
        <dl className="my-3 grid gap-2 sm:grid-cols-2">
          {Object.entries(step.state).map(([name, value]) => (
            <div key={name} className="rounded border border-edge p-2">
              <dt className="font-mono text-sm text-crab">{name}</dt>
              <dd className="mt-1 text-sm">{value}</dd>
            </div>
          ))}
        </dl>
        {step.output !== undefined && (
          <>
            <p className="eyebrow">Output so far</p>
            <pre className="whitespace-pre-wrap text-sm text-ok">
              {step.output || '(no output)'}
            </pre>
          </>
        )}
      </div>
    </section>
  )
}
