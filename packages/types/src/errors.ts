export type AppErrorCode =
  | 'VALIDATION_ERROR'
  | 'NOT_FOUND'
  | 'CONFLICT'
  | 'PERMISSION_DENIED'
  | 'IO_ERROR'
  | 'DB_ERROR'
  | 'MIGRATION_REQUIRED'
  | 'CORRUPT_VAULT'
  | 'HASH_MISMATCH'
  | 'EXPORT_FAILED'
  | 'IMPORT_FAILED'
  | 'LICENSE_INVALID'
  | 'UNSUPPORTED_FORMAT'
  | 'INTERNAL_ERROR';

export type AppErrorDto = {
  code: AppErrorCode;
  message: string;
  details?: unknown;
  retryable: boolean;
  user_action?: string;
};
