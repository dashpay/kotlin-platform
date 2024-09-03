package org.dashj.platform.sdk.base;

public abstract class Result<T, E> {
    private Result() {
    }

    public static <T, E> Result<T, E> Ok(T value) {
        return new Success<>(value);
    }

    public static <T, E> Result<T, E> Err(E error) {
        return new Failure<>(error);
    }

    public abstract T unwrap() throws Exception;

    public abstract E unwrapError() throws Exception;

    private static class Success<T, E> extends Result<T, E> {
        private final T value;

        private Success(T value) {
            this.value = value;
        }

        @Override
        public T unwrap() {
            return value;
        }

        @Override
        public E unwrapError() throws Exception {
            throw new Exception("Attempted to unwrapError on a Success: ");
        }

        @Override
        public String toString() {
            return String.format("Success{ %s }", value);
        }
    }

    private static class Failure<T, E> extends Result<T, E> {
        private final E error;

        private Failure(E error) {
            this.error = error;
        }

        @Override
        public T unwrap() throws Exception {
            throw new Exception("Attempted to unwrap a Failure: " + error);
        }

        @Override
        public E unwrapError() {
            return error;
        }

        @Override
        public String toString() {
            return String.format("Error{ %s }", error.toString());
        }
    }
}
