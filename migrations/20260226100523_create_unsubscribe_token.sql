CREATE TABLE unsubscribe_tokens(
   unsubscribe_token TEXT NOT NULL,
   subscriber_id uuid NOT NULL
      REFERENCES subscriptions (id),
   PRIMARY KEY (unsubscribe_token)
);
