INSERT INTO welcome (guild_id, channel_id, message)
                            VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO
                            UPDATE
                            SET channel_id = coalesce($2, welcome.channel_id),
                                message = coalesce($3, welcome.message) RETURNING *